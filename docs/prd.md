# ESP32-C3 0.42인치 OLED "highmaru" 표시 펌웨어 PRD

## 1. 개요

ESP32-C3에 연결된 0.42인치 I2C OLED에 `highmaru` 텍스트를 표시하는 Rust 펌웨어를 만든다. 프로젝트는 이후 센서 표시, 화면 전환, 폰트 교체, 보드 핀 변경이 쉬운 구조를 목표로 한다.

## 2. 목표

- COM10에 연결된 ESP32-C3에 플래시 가능한 Rust 펌웨어 제공
- Docker 기반 빌드 환경 제공
- 0.42인치 SSD1306 계열 OLED에 `highmaru` 텍스트 표시
- 하드웨어 설정, 보드 초기화, OLED 렌더링, 애플리케이션 흐름을 분리한 모듈 구조 제공
- I2C 핀, OLED 주소, 디스플레이 크기, 표시 문구를 변경하기 쉬운 구조 제공

## 3. 비목표

- Wi-Fi, Bluetooth, OTA 업데이트 구현
- 복잡한 UI 프레임워크 또는 애니메이션 구현
- OLED 모델 자동 탐지
- 보드별 핀맵 자동 판별
- Windows Docker 컨테이너 내부에서 COM10 직접 플래시 보장

## 4. 하드웨어 가정

- MCU: ESP32-C3
- 연결 포트: COM10
- OLED: 0.42인치 I2C OLED, SSD1306 컨트롤러 계열
- 라이브러리 호환성: u8g2의 SSD1306 72x40 계열 생성자와 호환되는 OLED로 가정
- 실제 표시 영역: 72 x 40
- SSD1306 주소 지정: 128 x 64 버퍼 내부의 `(30, 12)` offset 영역
- 기본 I2C 주소: `0x3C`
- 기본 I2C 핀:
  - SDA: GPIO5
  - SCL: GPIO6
- I2C 속도: 400 kHz

실제 배선이 다르면 `src/config.rs`의 설정값과 `src/board.rs`의 핀 매핑을 함께 수정한다.

u8g2 기준으로는 보통 `U8G2_SSD1306_128X64_NONAME_F_HW_I2C` 또는 유사 생성자를 사용하되, 128x64 버퍼 내부의 `(30, 12)` offset부터 72x40 영역만 실제 화면으로 본다. `docs/ref.md`의 참고 링크들은 온보드 OLED가 GPIO5/6에 연결되어 있고 I2C 주소가 `0x3C`라는 예제를 제공한다. Rust 구현에서는 C 기반 u8g2를 직접 바인딩하지 않고, 동일 SSD1306 컨트롤러를 대상으로 하는 `ssd1306` 크레이트와 `embedded-graphics`를 사용한다.

## 5. 사용자 시나리오

1. 개발자가 ESP32-C3를 USB로 PC에 연결한다.
2. OLED를 ESP32-C3의 I2C 핀에 연결한다.
3. `docker compose run --rm build`로 펌웨어를 빌드한다.
4. Windows 호스트에서 `espflash flash --monitor --port COM10 target/riscv32imc-esp-espidf/debug/highmaru-oled`로 보드에 업로드한다.
5. OLED 화면 중앙 근처에 `highmaru`가 표시된다.

## 6. 기능 요구사항

- FR-1: 부팅 시 I2C 버스를 초기화한다.
- FR-2: SSD1306 OLED를 초기화한다.
- FR-3: 화면 버퍼를 지운 뒤 `highmaru` 텍스트를 표시한다.
- FR-4: 표시 후 펌웨어는 계속 동작하며 재부팅되지 않아야 한다.
- FR-5: 표시 문구는 `src/config.rs`에서 변경 가능해야 한다.
- FR-6: OLED 관련 코드는 `src/display.rs`에 캡슐화한다.
- FR-7: 애플리케이션 흐름은 `src/app.rs`에 캡슐화한다.
- FR-8: 보드 주변장치와 핀 선택은 `src/board.rs`에 캡슐화한다.
- FR-9: Docker 빌드 명령은 `docker-compose.yml`에 캡슐화한다.

## 7. 비기능 요구사항

- Rust 기반으로 작성한다.
- ESP-IDF 기반 `std` 환경을 사용한다.
- Docker 빌드 환경은 로컬 Rust 설치에 의존하지 않아야 한다.
- `rust-toolchain.toml`은 upstream `stable`이 아니라 Espressif `esp` 채널을 사용한다.
- u8g2 호환성은 디스플레이 컨트롤러와 해상도 식별 기준으로 사용한다.
- 모듈 경계가 명확해야 한다.
- 하드웨어 값은 매직 넘버로 흩어지지 않아야 한다.
- 실패 시 `anyhow::Result`로 오류를 상위로 전달한다.

## 8. 모듈 설계

### `src/config.rs`

하드웨어 및 앱 설정을 보관한다.

- `OLED_I2C_ADDRESS`
- `I2C_BAUDRATE_HZ`
- `DISPLAY_TEXT`
- `SDA_PIN`, `SCL_PIN`
- `VIEWPORT_WIDTH`, `VIEWPORT_HEIGHT`
- `VIEWPORT_X_OFFSET`, `VIEWPORT_Y_OFFSET`

### `src/board.rs`

ESP32-C3 주변장치와 핀 매핑을 담당한다.

- `Peripherals` 기반 I2C 드라이버 생성
- 기본 SDA/SCL 핀 매핑 적용
- 핀 변경 시 수정 지점 최소화

### `src/display.rs`

OLED 초기화와 그리기 책임을 가진다.

- SSD1306 드라이버 생성
- 디스플레이 초기화
- 화면 지우기
- 128x64 버퍼 안의 72x40 viewport 기준 좌표 변환
- 텍스트 렌더링
- 화면 flush

### `src/app.rs`

펌웨어의 상위 실행 흐름을 가진다.

- `OledDisplay` 생성
- 초기 화면 렌더링
- 메인 루프 유지

### `src/main.rs`

하드웨어 진입점이다.

- ESP-IDF 패치 연결
- `Peripherals` 획득
- `board::create_i2c_driver` 호출
- `app::run` 호출

## 9. Docker 설계

### `Dockerfile`

- `espressif/idf-rust:all_latest` 이미지를 기반으로 사용한다.
- 작업 디렉터리는 `/workspace`로 고정한다.
- 기본 명령은 `cargo build`로 둔다.

### `docker-compose.yml`

- `build` 서비스: 디버그 빌드 수행
- `release` 서비스: 릴리스 빌드 수행
- Cargo registry와 git cache는 Docker volume으로 분리해 반복 빌드 시간을 줄인다.
- 프로젝트 디렉터리는 bind mount로 연결해 산출물이 호스트 `target` 디렉터리에 남도록 한다.

## 10. 빌드 및 플래시 계획

### Docker 빌드

```powershell
docker compose run --rm build
```

### Docker 릴리스 빌드

```powershell
docker compose run --rm release
```

### 로컬 빌드

```powershell
cargo build
```

### COM10 플래시 및 모니터

```powershell
espflash flash --monitor --port COM10 target/riscv32imc-esp-espidf/debug/highmaru-oled
```

`espflash`가 설치되어 있지 않으면 Windows 호스트에서 다음 명령으로 설치한다.

```powershell
cargo install espflash
```

## 11. 검증 기준

- `docker compose run --rm build`가 성공한다.
- COM10 플래시가 성공한다.
- OLED 화면에 `highmaru`가 표시된다.
- 펌웨어가 표시 후 panic 또는 reset loop에 빠지지 않는다.

## 12. 향후 확장

- 표시 텍스트를 여러 줄로 확장
- 버튼 입력을 받아 화면 전환
- 센서 값을 OLED에 주기적으로 표시
- 디스플레이 크기별 레이아웃 전략 분리
- SPI OLED 지원 추가
