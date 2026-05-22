#[cfg(all(
    feature = "board-original",
    any(
        feature = "board-heltec",
        feature = "board-esp32-cam",
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    )
))]
compile_error!(
    "Select exactly one board feature: board-original, board-heltec, board-esp32-cam, board-esp32-p4-pico-kit-a, or board-esp32-p4-wifi6-kit-a."
);

#[cfg(all(
    feature = "board-heltec",
    any(
        feature = "board-esp32-cam",
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    )
))]
compile_error!(
    "Select exactly one board feature: board-original, board-heltec, board-esp32-cam, board-esp32-p4-pico-kit-a, or board-esp32-p4-wifi6-kit-a."
);

#[cfg(all(
    feature = "board-esp32-cam",
    any(
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    )
))]
compile_error!(
    "Select exactly one board feature: board-original, board-heltec, board-esp32-cam, board-esp32-p4-pico-kit-a, or board-esp32-p4-wifi6-kit-a."
);

#[cfg(all(
    feature = "board-esp32-p4-pico-kit-a",
    feature = "board-esp32-p4-wifi6-kit-a"
))]
compile_error!(
    "Select exactly one board feature: board-original, board-heltec, board-esp32-cam, board-esp32-p4-pico-kit-a, or board-esp32-p4-wifi6-kit-a."
);

#[cfg(not(any(
    feature = "board-original",
    feature = "board-heltec",
    feature = "board-esp32-cam",
    feature = "board-esp32-p4-pico-kit-a",
    feature = "board-esp32-p4-wifi6-kit-a"
)))]
compile_error!(
    "Select one board feature: board-original, board-heltec, board-esp32-cam, board-esp32-p4-pico-kit-a, or board-esp32-p4-wifi6-kit-a."
);

#[cfg(all(
    feature = "camera",
    not(any(
        feature = "board-esp32-cam",
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    ))
))]
compile_error!(
    "The camera feature requires board-esp32-cam, board-esp32-p4-pico-kit-a, or board-esp32-p4-wifi6-kit-a."
);

#[cfg(all(
    feature = "audio",
    not(any(
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    ))
))]
compile_error!("The audio feature requires board-esp32-p4-pico-kit-a or board-esp32-p4-wifi6-kit-a.");

#[cfg(all(feature = "wifi", not(feature = "board-esp32-p4-wifi6-kit-a")))]
compile_error!("The wifi feature requires board-esp32-p4-wifi6-kit-a.");

#[cfg(all(feature = "lora", not(feature = "board-heltec")))]
compile_error!("The lora feature requires board-heltec.");

pub const OLED_I2C_ADDRESS: u8 = 0x3c;
pub const I2C_BAUDRATE_HZ: u32 = 400_000;

#[cfg(feature = "board-original")]
pub const BOARD_NAME: &str = "original ESP32-C3 OLED";
#[cfg(feature = "board-original")]
pub const HAS_CAMERA: bool = false;
#[cfg(feature = "board-original")]
pub const HAS_AUDIO: bool = false;
#[cfg(feature = "board-original")]
pub const HAS_WIFI: bool = false;
#[cfg(feature = "board-original")]
pub const HAS_LORA: bool = false;
#[cfg(feature = "board-original")]
pub const HAS_OLED: bool = true;
#[cfg(feature = "board-original")]
pub const OLED_RESET_PIN: Option<i32> = None;
#[cfg(feature = "board-original")]
pub const OLED_POWER_PIN: Option<i32> = None;
#[cfg(feature = "board-original")]
pub const OLED_ROTATION_DEGREES: u16 = 0;
#[cfg(feature = "board-original")]
pub const SDA_PIN: i32 = 5;
#[cfg(feature = "board-original")]
pub const SCL_PIN: i32 = 6;
#[cfg(feature = "board-original")]
pub const VIEWPORT_WIDTH: i32 = 72;
#[cfg(feature = "board-original")]
pub const VIEWPORT_HEIGHT: i32 = 40;
#[cfg(feature = "board-original")]
pub const VIEWPORT_X_OFFSET: i32 = 28;
#[cfg(feature = "board-original")]
pub const VIEWPORT_Y_OFFSET: i32 = 24;

#[cfg(feature = "board-heltec")]
pub const BOARD_NAME: &str = "Heltec WiFi LoRa 32 V2";
#[cfg(feature = "board-heltec")]
pub const HAS_CAMERA: bool = false;
#[cfg(feature = "board-heltec")]
pub const HAS_AUDIO: bool = false;
#[cfg(feature = "board-heltec")]
pub const HAS_WIFI: bool = false;
#[cfg(feature = "board-heltec")]
pub const HAS_LORA: bool = true;
#[cfg(feature = "board-heltec")]
pub const HAS_OLED: bool = true;
#[cfg(feature = "board-heltec")]
pub const OLED_RESET_PIN: Option<i32> = Some(16);
#[cfg(feature = "board-heltec")]
pub const OLED_POWER_PIN: Option<i32> = Some(21);
#[cfg(feature = "board-heltec")]
pub const OLED_ROTATION_DEGREES: u16 = 180;
#[cfg(feature = "board-heltec")]
pub const SDA_PIN: i32 = 4;
#[cfg(feature = "board-heltec")]
pub const SCL_PIN: i32 = 15;
#[cfg(feature = "board-heltec")]
pub const VIEWPORT_WIDTH: i32 = 128;
#[cfg(feature = "board-heltec")]
pub const VIEWPORT_HEIGHT: i32 = 64;
#[cfg(feature = "board-heltec")]
pub const VIEWPORT_X_OFFSET: i32 = 0;
#[cfg(feature = "board-heltec")]
pub const VIEWPORT_Y_OFFSET: i32 = 0;

#[cfg(feature = "board-esp32-cam")]
pub const BOARD_NAME: &str = "ESP32-CAM external OLED";
#[cfg(feature = "board-esp32-cam")]
pub const HAS_CAMERA: bool = true;
#[cfg(feature = "board-esp32-cam")]
pub const HAS_AUDIO: bool = false;
#[cfg(feature = "board-esp32-cam")]
pub const HAS_WIFI: bool = false;
#[cfg(feature = "board-esp32-cam")]
pub const HAS_LORA: bool = false;
#[cfg(feature = "board-esp32-cam")]
pub const HAS_OLED: bool = true;
#[cfg(feature = "board-esp32-cam")]
pub const OLED_RESET_PIN: Option<i32> = None;
#[cfg(feature = "board-esp32-cam")]
pub const OLED_POWER_PIN: Option<i32> = None;
#[cfg(feature = "board-esp32-cam")]
pub const OLED_ROTATION_DEGREES: u16 = 0;
#[cfg(feature = "board-esp32-cam")]
pub const SDA_PIN: i32 = 14;
#[cfg(feature = "board-esp32-cam")]
pub const SCL_PIN: i32 = 15;
#[cfg(feature = "board-esp32-cam")]
pub const VIEWPORT_WIDTH: i32 = 128;
#[cfg(feature = "board-esp32-cam")]
pub const VIEWPORT_HEIGHT: i32 = 64;
#[cfg(feature = "board-esp32-cam")]
pub const VIEWPORT_X_OFFSET: i32 = 0;
#[cfg(feature = "board-esp32-cam")]
pub const VIEWPORT_Y_OFFSET: i32 = 0;

#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const BOARD_NAME: &str = "Waveshare ESP32-P4-Pico-KIT-A";
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const HAS_CAMERA: bool = true;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const HAS_AUDIO: bool = true;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const HAS_WIFI: bool = false;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const HAS_LORA: bool = false;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const HAS_OLED: bool = false;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const OLED_RESET_PIN: Option<i32> = None;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const OLED_POWER_PIN: Option<i32> = None;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const OLED_ROTATION_DEGREES: u16 = 0;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const SDA_PIN: i32 = 7;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const SCL_PIN: i32 = 8;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const VIEWPORT_WIDTH: i32 = 0;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const VIEWPORT_HEIGHT: i32 = 0;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const VIEWPORT_X_OFFSET: i32 = 0;
#[cfg(feature = "board-esp32-p4-pico-kit-a")]
pub const VIEWPORT_Y_OFFSET: i32 = 0;

#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const BOARD_NAME: &str = "Waveshare ESP32-P4-WIFI6-KIT-A";
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const HAS_CAMERA: bool = true;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const HAS_AUDIO: bool = true;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const HAS_WIFI: bool = true;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const HAS_LORA: bool = false;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const HAS_OLED: bool = false;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const OLED_RESET_PIN: Option<i32> = None;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const OLED_POWER_PIN: Option<i32> = None;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const OLED_ROTATION_DEGREES: u16 = 0;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const SDA_PIN: i32 = 7;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const SCL_PIN: i32 = 8;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const VIEWPORT_WIDTH: i32 = 0;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const VIEWPORT_HEIGHT: i32 = 0;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const VIEWPORT_X_OFFSET: i32 = 0;
#[cfg(feature = "board-esp32-p4-wifi6-kit-a")]
pub const VIEWPORT_Y_OFFSET: i32 = 0;

pub const DISPLAY_TEXT: &str = "highmaru";

#[cfg(all(feature = "board-heltec", feature = "lora"))]
#[allow(dead_code)]
pub mod lora {
    pub const CHIP: &str = "SX1276/SX1278";
    pub const SCK_PIN: i32 = 5;
    pub const MISO_PIN: i32 = 19;
    pub const MOSI_PIN: i32 = 27;
    pub const NSS_PIN: i32 = 18;
    pub const RESET_PIN: i32 = 14;
    pub const DIO0_PIN: i32 = 26;
    pub const DIO1_PIN: i32 = 35;
    pub const DIO2_PIN: i32 = 34;
}

#[cfg(all(feature = "board-esp32-cam", feature = "camera"))]
#[allow(dead_code)]
pub mod camera {
    pub const MODEL: &str = "OV2640";
    pub const PWDN_PIN: i32 = 32;
    pub const RESET_PIN: i32 = -1;
    pub const XCLK_PIN: i32 = 0;
    pub const SIOD_PIN: i32 = 26;
    pub const SIOC_PIN: i32 = 27;
    pub const Y9_PIN: i32 = 35;
    pub const Y8_PIN: i32 = 34;
    pub const Y7_PIN: i32 = 39;
    pub const Y6_PIN: i32 = 36;
    pub const Y5_PIN: i32 = 21;
    pub const Y4_PIN: i32 = 19;
    pub const Y3_PIN: i32 = 18;
    pub const Y2_PIN: i32 = 5;
    pub const VSYNC_PIN: i32 = 25;
    pub const HREF_PIN: i32 = 23;
    pub const PCLK_PIN: i32 = 22;
}

#[cfg(all(
    any(
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    ),
    feature = "audio"
))]
#[allow(dead_code)]
pub mod audio {
    pub const CODEC: &str = "ES8311";
    pub const AMPLIFIER: &str = "NS4150B";
    pub const CODEC_I2C_ADDRESS: u8 = 0x18;
    pub const I2C_SDA_PIN: i32 = 7;
    pub const I2C_SCL_PIN: i32 = 8;
    pub const I2S_MCLK_PIN: i32 = 13;
    pub const I2S_SCLK_PIN: i32 = 12;
    pub const I2S_DOUT_PIN: i32 = 11;
    pub const I2S_LRCK_PIN: i32 = 10;
    pub const I2S_DIN_PIN: i32 = 9;
    pub const PA_ENABLE_PIN: i32 = 53;
}

#[cfg(all(
    any(
        feature = "board-esp32-p4-pico-kit-a",
        feature = "board-esp32-p4-wifi6-kit-a"
    ),
    feature = "camera"
))]
#[allow(dead_code)]
pub mod camera {
    pub const INTERFACE: &str = "MIPI-CSI 2-lane";
    pub const KIT_MODULE: &str = "RPi Camera (B)";
    pub const COMPATIBLE_SENSOR: &str = "OV5647";
}

#[cfg(all(feature = "board-esp32-p4-wifi6-kit-a", feature = "wifi"))]
#[allow(dead_code)]
pub mod wifi {
    pub const COPROCESSOR: &str = "ESP32-C6-MINI-1";
    pub const HOST_INTERFACE: &str = "SDIO";
    pub const RADIO: &str = "Wi-Fi 6 2.4 GHz + BLE 5";
}
