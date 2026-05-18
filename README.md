# highmaru-oled

ESP32-C3와 0.42인치 SSD1306 I2C OLED에 `highmaru`를 표시하는 Rust 펌웨어입니다.

## 기본 설정

- Port: COM10
- OLED I2C address: `0x3C`
- SDA: GPIO5
- SCL: GPIO6
- Visible display: 72 x 40
- SSD1306 addressing: 128 x 64 buffer with visible viewport offset `(30, 12)`
- u8g2 equivalent: `U8G2_SSD1306_72X40_ER_F_HW_I2C` 계열

u8g2 호환 OLED지만 Rust 코드에서는 C u8g2 바인딩 대신 `ssd1306` + `embedded-graphics`를 사용합니다. 참고 링크의 예제처럼 128x64 SSD1306 버퍼 안의 `(30, 12)` 위치부터 72x40 영역만 실제 화면으로 보고 렌더링합니다.

`docs/ref.md`의 참고 링크 기준으로 이 보드의 온보드 OLED는 일반 ESP32-C3 기본 I2C(GPIO8/9)가 아니라 GPIO5/6에 연결된 것으로 봅니다.

핀 또는 표시 문구를 바꾸려면 `src/config.rs`와 `src/board.rs`를 수정하세요.

## Docker Build

Windows 호스트에 Rust/ESP-IDF를 직접 설치하지 않고 Docker에서 빌드할 수 있습니다.

```powershell
docker compose run --rm build
```

첫 실행은 `espressif/idf-rust:all_latest` 이미지를 내려받기 때문에 오래 걸릴 수 있습니다.
이 프로젝트는 `riscv32imc-esp-espidf` 표준 라이브러리가 포함된 Espressif Rust 툴체인이 필요하므로 `rust-toolchain.toml`에서 `esp` 채널을 사용합니다.
ESP-IDF용 Rust target은 사전 설치 target이 아니라 Cargo의 `build-std` 기능으로 `std`와 `panic_abort`를 빌드합니다.

릴리스 빌드는 다음 명령을 사용합니다.

```powershell
docker compose run --rm release
```

빌드 산출물은 프로젝트의 `target` 디렉터리에 생성됩니다.

## Local Build

로컬에 ESP-RS 빌드 환경이 준비되어 있다면 직접 빌드할 수도 있습니다.

```powershell
cargo build
```

## Flash

Windows Docker Desktop에서는 컨테이너에서 `COM10`을 직접 다루기 어렵습니다. 빌드는 Docker에서 수행하고, 플래시는 Windows 호스트에서 실행하는 방식을 권장합니다.

```powershell
espflash flash --monitor --port COM10 target/riscv32imc-esp-espidf/debug/highmaru-oled
```
