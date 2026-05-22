# highmaru-oled

ESP32 계열 보드와 SSD1306 I2C OLED에 `highmaru`를 표시하는 Rust 펌웨어입니다.

## 보드 설정 선택

Cargo feature로 보드별 I2C 핀과 표시 영역을 선택합니다. feature를 지정하지 않으면 기존 ESP32-C3 0.42인치 OLED 설정(`board-original`)이 사용됩니다.

| Board feature | Function feature | Target | SDA | SCL | Reset | OLED viewport | 추가 기능 |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `board-original` | none | `riscv32imc-esp-espidf` | GPIO5 | GPIO6 | none | 72 x 40 at `(28, 24)` | none |
| `board-heltec` | `lora` | `xtensa-esp32-espidf` | GPIO4 | GPIO15 | GPIO16 | 128 x 64 at `(0, 0)` | SX1276/SX1278 LoRa |
| `board-esp32-cam` | `camera` | `xtensa-esp32-espidf` | GPIO14 | GPIO15 | none | 128 x 64 at `(0, 0)` | OV2640 camera |
| `board-esp32-p4-pico-kit-a` | `audio,camera` | `riscv32imafc-esp-espidf` | GPIO7 | GPIO8 | none | none | ES8311 audio, RPi Camera(B) over MIPI-CSI |

기능 feature는 보드 capability 검사를 통과해야 합니다. `camera`는 `board-esp32-cam` 또는 `board-esp32-p4-pico-kit-a`에서만, `audio`는 `board-esp32-p4-pico-kit-a`에서만, `lora`는 `board-heltec`에서만 사용할 수 있습니다. 예를 들어 `board-original,lora` 또는 `board-heltec,camera` 조합은 빌드 단계에서 실패합니다.

보드 선택 예시는 다음과 같습니다.

```powershell
# 기존 설정
cargo build
cargo build --features board-original

# Heltec
cargo build --target xtensa-esp32-espidf --no-default-features --features board-heltec,lora

# ESP32-CAM
cargo build --target xtensa-esp32-espidf --no-default-features --features board-esp32-cam,camera

# Waveshare ESP32-P4-Pico-KIT-A
cargo build --target riscv32imafc-esp-espidf --no-default-features --features board-esp32-p4-pico-kit-a,audio,camera
```

Docker 빌드는 보드별 서비스를 사용할 수 있습니다.

```powershell
docker compose run --rm build
docker compose run --rm build-heltec
docker compose run --rm build-esp32-cam
docker compose run --rm build-esp32-p4-pico-kit-a
```

현재 연결된 Heltec V2는 이 환경에서 `/dev/cu.usbserial-0001`와 `/dev/tty.usbserial-0001`로 보입니다. 일반적으로 macOS에서는 플래시에 `/dev/cu.usbserial-0001`를 사용합니다.

```zsh
espflash flash --monitor --port /dev/cu.usbserial-0001 target/xtensa-esp32-espidf/debug/highmaru-oled
```

Heltec WiFi LoRa 32 V2는 SX1276/SX1278 LoRa 칩을 포함합니다. 이 펌웨어는 `board-heltec` 선택 시 OLED 초기화 전에 GPIO16을 low/high로 토글하고, `lora` 기능은 Heltec 보드에서만 켤 수 있게 제한합니다.

ESP32-CAM은 OV2640 카메라 모듈을 전제로 합니다. `camera` 기능은 ESP32-CAM 보드에서만 켤 수 있게 제한합니다.

Waveshare ESP32-P4-Pico-KIT-A는 온보드 SSD1306 OLED가 없으므로 이 펌웨어에서는 OLED 초기화를 건너뛰고 보드/오디오/카메라 설정을 로그로 출력한 뒤 유지됩니다. Kit-A 구성품의 RPi Camera(B)는 MIPI-CSI 2-lane 포트를 사용하며, 스피커는 ES8311 Codec + NS4150B 앰프 경로를 사용합니다. ESP32-P4-Pico의 기본 I2C는 SDA GPIO7, SCL GPIO8이고 ES8311 Codec 주소는 `0x18`입니다.

P4 Kit-A 오디오 핀 정의:

| Signal | GPIO |
| --- | ---: |
| MCLK | GPIO13 |
| SCLK | GPIO12 |
| ASDOUT / DOUT | GPIO11 |
| LRCK | GPIO10 |
| DSDIN / DIN | GPIO9 |
| PA enable | GPIO53 |

## 기존 기본 설정

- Port: COM10
- OLED I2C address: `0x3C`
- SDA: GPIO5
- SCL: GPIO6
- Visible display: 72 x 40
- SSD1306 addressing: 128 x 64 buffer with visible viewport offset `(28, 24)`
- u8g2 equivalent: `U8G2_SSD1306_72X40_ER_F_HW_I2C` 계열

u8g2 호환 OLED지만 Rust 코드에서는 C u8g2 바인딩 대신 `ssd1306` + `embedded-graphics`를 사용합니다. 참고 링크의 예제처럼 128x64 SSD1306 버퍼 안의 `(28, 24)` 위치부터 72x40 영역만 실제 화면으로 보고 렌더링합니다.

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

ESP32-P4-Pico-KIT-A가 Windows에서 `COM12`로 보이면 다음 경로를 사용합니다.

```powershell
# espflash flash --monitor --port COM12 target/riscv32imafc-esp-espidf/debug/highmaru-oled
# ../../espflash.exe flash --chip esp32p4 --before no-reset --no-stub --baud 115200 --port COM12 target/riscv32imafc-esp-espidf/debug/highmaru-oled
# ../../espflash.exe flash --chip esp32p4 --before default-reset --no-stub --baud 115200 --port COM12 target/riscv32imafc-esp-espidf/debug/highmaru-oled
../../espflash.exe flash --monitor --chip esp32p4 --before default-reset --no-stub --baud 115200 --port COM12 target/riscv32imafc-esp-espidf/debug/highmaru-oled
```

macOS에서는 `/dev/cu.*` 포트를 확인한 뒤 다음처럼 플래시합니다.

```zsh
espflash flash --monitor --port /dev/cu.usbmodemXXXX target/riscv32imc-esp-espidf/debug/highmaru-oled
```

릴리스 빌드라면 다음 경로의 산출물을 사용합니다.

```zsh
espflash flash --monitor --port /dev/cu.usbmodemXXXX target/riscv32imc-esp-espidf/release/highmaru-oled
```
