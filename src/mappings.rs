use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "93";

pub const ROW_COUNT: usize = 3;
pub const COL_COUNT: usize = 5;
pub const KEY_COUNT: usize = ROW_COUNT * COL_COUNT;
pub const ENCODER_COUNT: usize = 0;

pub const IMAGE_FORMAT: ImageFormat = ImageFormat {
    mode: ImageMode::JPEG,
    size: (100, 100),
    rotation: ImageRotation::Rot180,
    mirror: ImageMirroring::None,
};

#[derive(Debug, Clone)]
pub enum Kind {
    Mirabox293,
}

pub const MIRABOX293_VID: u16 = 0x5500;

pub const MIRABOX293_PID: u16 = 0x1001;

// Map all queries to usage page 65440 and usage id 1 for now
pub const MIRABOX293_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, MIRABOX293_VID, MIRABOX293_PID);

pub const QUERIES: [DeviceQuery; 1] = [MIRABOX293_QUERY];

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match vid {
            MIRABOX293_VID => match pid {
                MIRABOX293_PID => Some(Kind::Mirabox293),
                _ => None,
            },

            _ => None,
        }
    }

    /// Returns true for devices that emitting two events per key press, instead of one
    /// Currently none of the devices from this family support that
    pub fn supports_both_states(&self) -> bool {
        true
    }

    pub fn is_v2(&self) -> bool {
        false // In the future there may be "v2" devices, so lay some groundwork
    }

    /// There is no point relying on manufacturer/device names reported by the USB stack,
    /// so we return custom names for all the kinds of devices
    pub fn human_name(&self) -> String {
        match &self {
            Self::Mirabox293 => "Mirabox 293",
        }
        .to_string()
    }

    /// Because "v1" devices all share the same serial number, use custom suffix to be able to connect
    /// two devices with the different revisions at the same time
    pub fn id_suffix(&self) -> String {
        match &self {
            Self::Mirabox293 => "M293",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
