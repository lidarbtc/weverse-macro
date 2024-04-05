# 위버스 공방/사녹/폼림 클릭 매크로

이 프로그램은 사용자가 지정한 시간에 자동으로 마우스 좌클릭을 수행합니다. 매우 빠르게 동작하여 정확한 타이밍에 클릭이 필요한 순간에 유용하게 사용될 수 있습니다. 하지만, 성공은 운에 크게 좌우됨을 기억해주세요. 네트워크 환경에 의해 좌우되는 부분이 많습니다. 또한 최대 2ms의 오차가 발생할 수 있습니다.

## 빌드 환경 구축 방법

각 운영 체제별로 필요한 라이브러리와 도구를 설치하는 방법입니다.

### Linux

```bash
sudo apt install libxdo-dev libgtk-4-dev build-essential
```

### macOS

```bash
brew install gtk4
```

### Windows

Windows 사용자는 [GTK4 공식 문서](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)를 참조하여 필요한 구성 요소를 설치해주세요.

## 프로그램 빌드 방법

아래의 명령어를 사용하여 프로그램을 빌드합니다. 이 과정을 통해 `target/release` 폴더에 실행 파일이 생성됩니다.

```bash
cargo build --release
```

## 사용 방법

1. 위의 지시사항에 따라 빌드 환경을 준비합니다.
2. 프로그램을 빌드합니다.
3. 실행 파일을 사용하여 원하는 시간에 자동 클릭을 설정합니다.

## Good Luck!

이 매크로를 사용하여 공방 추첨에 성공하시길 바랍니다. 모든 것은 운과 타이밍에 달려 있습니다!
