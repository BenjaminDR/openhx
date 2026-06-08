/// Static metadata that fully describes a supported Line 6 device from the
/// perspective of the USB transport and the preset-read protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceProfile {
    pub name: &'static str,
    pub vendor_id: u16,
    pub product_id: u16,
    pub preset_count: u16,
}

impl DeviceProfile {
    /// Returns the 3-byte MessagePack `array16` marker that corresponds to
    /// [`Self::preset_count`].
    #[inline]
    pub fn array_marker(&self) -> [u8; 3] {
        let [hi, lo] = self.preset_count.to_be_bytes();
        [0xDC, hi, lo]
    }
}

impl std::fmt::Display for DeviceProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (VID={:#06X} PID={:#06X}, {} presets)",
            self.name, self.vendor_id, self.product_id, self.preset_count,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Preset;

    // DeviceProfile

    #[test]
    fn device_profile_array_marker_generation() {
        let profile = DeviceProfile {
            name: "Mock Device",
            vendor_id: 0x1234,
            product_id: 0x5678,
            preset_count: 258, // 0x0102 in hex
        };
        assert_eq!(profile.array_marker(), [0xDC, 0x01, 0x02]);
    }

    #[test]
    fn device_profile_display_format() {
        let profile = DeviceProfile {
            name: "HX Stomp",
            vendor_id: 0x0E41,
            product_id: 0x4252,
            preset_count: 126,
        };
        assert_eq!(
            profile.to_string(),
            "HX Stomp (VID=0x0E41 PID=0x4252, 126 presets)"
        );
    }

    // Preset

    #[test]
    fn preset_creation_stores_fields() {
        let p = Preset::new(7, "Blues Drive");
        assert_eq!(p.index, 7);
        assert_eq!(p.name, "Blues Drive");
    }

    #[test]
    fn preset_display_handles_padding() {
        assert_eq!(Preset::new(1, "A").to_string(), "  1: A");
        assert_eq!(Preset::new(42, "B").to_string(), " 42: B");
        assert_eq!(Preset::new(127, "C").to_string(), "127: C");
    }

    #[test]
    fn presets_sort_by_index_then_name() {
        let mut presets = [
            Preset::new(10, "Zebra"),
            Preset::new(0, "Alpha"),
            Preset::new(10, "Beta"),
        ];
        presets.sort_unstable();

        assert_eq!(presets[0].name, "Alpha"); // index 0
        assert_eq!(presets[1].name, "Beta"); // index 10, B
        assert_eq!(presets[2].name, "Zebra"); // index 10, Z
    }
}
