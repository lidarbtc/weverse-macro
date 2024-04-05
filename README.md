# 위버스 공방/사녹/폼림 클릭 매크로

```
작동 원리: 프로그램에서 시간을 지정하면 프로그램은 그 시간에 맞춰 마우스 좌클릭을 수행합니다.

매우 빠르게 작동하지만 결국 운이 가장 중요합니다.

최대 2ms의 오차가 발생할 수 있습니다.

아래는 각 시스템별 빌드 환경 구축 방법입니다.
linux:

sudo apt install libxdo-dev libgtk-4-dev build-essential

mac:

brew install gtk4

windows:

[gtk4 공식 문서](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)를 참조해주세요.

아래는 모든 시스템에서 동일한 빌드 방법입니다.

cargo build --release

위 명령어를 실행하면 target/release 폴더에 실행 파일이 생성됩니다.

Good luck...
```
