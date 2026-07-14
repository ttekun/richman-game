# Handoff Prompt: Code Review → Commit → Push → Deploy

## Copy-paste this entire message into a new Hermes session:

---

richman-game 프로젝트(/home/dev/richman-game)의 코드 리뷰, 커밋, push, 배포를 해주세요. 아래 지시에 정확히 따라주세요.

## 프로젝트 개요

- **위치**: `/home/dev/richman-game/`
- **내용**: 20億円の夢 — Rich Man Simulator (경제 시뮬레이션 게임)
- **변경 내용**: JS 인라인 게임 로직을 Rust/WASM으로 포팅 + 3개 언어 i18n (ko/ja/en) 추가
- **리포**: `origin http://github.com/ttekun/richman-game.git`
- **브랜치**: `main`
- **배포**: GitHub Pages (GitHub Actions 워크플로 사용)

## 현재 git 상태

```
Modified:
  .github/workflows/deploy.yml  (Rust 빌드 스텝 추가)
  index.html                    (WASM 기반으로 재작성 + i18n)

Untracked:
  docs/plans/                   (구현 계획서)
  wasm/                         (Rust 프로젝트 — src/, Cargo.toml, pkg/)
```

## 작업 순서

### Step 1: .gitignore 작성

`/home/dev/richman-game/wasm/.gitignore` 파일을 생성:

```
target/
```

`wasm/target/` 디렉토리는 224MB이므로 절대 커밋하지 않는다. `wasm/pkg/`은 GitHub Pages 배포에 필요하므로 커밋한다 (이미 `wasm/pkg/.gitignore`에 `*`가 있으므로 `git add -f` 필요).

### Step 2: 코드 리뷰

`requesting-code-review` 스킬을 로드하고 실행:

1. `skill_view(name='requesting-code-review')` 로드
2. 스킬 지시에 따라 코드 리뷰 수행
3. 주요 검증 항목:
   - **Rust 코드**: `wasm/src/*.rs` (10개 파일, 1,720줄) — 컴파일 에러, 논리 오류, 안전성
   - **index.html**: 1,478줄 — JS 에러, i18n 누락 키, WASM 로딩 실패 처리
   - **deploy.yml**: GitHub Actions 워크플로 — Rust 설치, wasm-pack 빌드, Pages 배포
   - **.gitignore**: `wasm/target/` 제외 확인
4. 발견된 문제가 있으면 수정
5. 리뷰 결과를 요약해서 보고

### Step 3: 로컬 빌드 검증

Rust 코드가 정상적으로 빌드되는지 확인:

```bash
cd /home/dev/richman-game/wasm
cargo check 2>&1
```

에러가 있으면 수정. 그 후 WASM 재빌드:

```bash
cd /home/dev/richman-game/wasm
wasm-pack build --target web --release 2>&1
```

`wasm/pkg/` 디렉토리에 `.js`, `.wasm`, `.d.ts` 파일이 생성되는지 확인.

### Step 4: 브라우저 테스트

로컬 서버로 게임이 정상 동작하는지 확인:

```bash
cd /home/dev/richman-game && python3 -m http.server 8082
```

브라우저로 `http://localhost:8082` 접속 후:
1. 타이틀 화면 표시 확인
2. 🌐 버튼으로 언어 전환 (ko → ja → en) 확인
3. 게임 시작 → 설정 → 결정 → 연도 결과 화면 확인
4. 콘솔 에러 유무 확인

서버는 테스트 후 반드시 종료.

### Step 5: 커밋

수정이 모두 끝나면 커밋:

```bash
cd /home/dev/richman-game
git add .gitignore wasm/.gitignore .github/workflows/deploy.yml index.html wasm/Cargo.toml wasm/src/ docs/
git add -f wasm/pkg/
git commit -m "feat: port game logic to Rust/WASM + add 3-language i18n

- Port all game logic from inline JS to Rust (1,720 lines, 10 files)
- Build to WASM via wasm-pack --target web
- GameEngine struct with process_year, get_decisions, process_exit, etc.
- Injectable RNG trait (JsRng for browser, SeededRng for tests)
- Add 3-language i18n (ko/ja/en) via JS-layer translation maps
- Rust returns translation keys, JS resolves via t() function
- Language switcher button (🌐) with localStorage persistence
- Update GitHub Actions deploy workflow with Rust build steps"
```

주의:
- `wasm/target/`이 커밋에 포함되지 않도록 `.gitignore` 확인
- `wasm/pkg/`은 `git add -f`로 강제 추가 (pkg/.gitignore에 `*`가 있어 무시됨)

### Step 6: Push

```bash
cd /home/dev/richman-game
git push origin main
```

### Step 7: 배포 확인

1. GitHub Actions 워크플로 실행 확인:
   - `https://github.com/ttekun/richman-game/actions` 에서 최신 워크플로 상태 확인
   - 워크플로가 완료될 때까지 대기 (Rust 설치 + wasm-pack 빌드 + Pages 배포, 약 3-5분)

2. GitHub Pages 사이트 확인:
   - 배포 URL: `https://ttekun.github.io/richman-game/`
   - 게임이 정상 로딩되는지 확인
   - WASM 파일이 로드되는지 브라우저 개발자 도구 Network 탭에서 확인
   - 3개 언어 전환이 동작하는지 확인

3. 문제가 발생하면:
   - Actions 로그 확인 → 빌드 에러 특정 → 수정 → 재푸시
   - Pages에서 WASM MIME 타입 에러 시 → `wasm/pkg/`이 커밋에 포함되었는지 확인

## 파일 구조

```
richman-game/
├── index.html              # UI rendering + i18n (1,478 lines)
├── .github/workflows/
│   └── deploy.yml          # CI/CD: Rust build + wasm-pack + Pages deploy
├── docs/plans/
│   └── 2026-07-14-wasm-port.md  # Implementation plan
└── wasm/
    ├── .gitignore           # target/ 제외
    ├── Cargo.toml           # Rust package config
    ├── src/                 # Rust 소스 (10 files, 1,720 lines)
    │   ├── lib.rs           # wasm-bindgen exports
    │   ├── state.rs         # GameState + sub-structs
    │   ├── rng.rs           # Rng trait, JsRng, SeededRng
    │   ├── constants.rs     # TOKYO_AREAS, GLOSSARY
    │   ├── format.rs        # format_yen, format_percent
    │   ├── events.rs        # 13 event scenarios
    │   ├── decisions.rs     # 21 decisions + apply_decision
    │   ├── engine.rs        # Year processing engine
    │   └── tax.rs           # Exit/tax/rank calculations
    └── pkg/                 # wasm-pack 빌드 결과물 (커밋 필요)
        ├── rich_man_engine.js
        ├── rich_man_engine_bg.wasm
        ├── rich_man_engine.d.ts
        └── rich_man_engine_bg.wasm.d.ts
```

## 주의사항

1. **wasm/target/ 커밋 금지** — 224MB, 빌드 캐시. `.gitignore`로 제외.
2. **wasm/pkg/ 커밋 필수** — GitHub Pages 배포에 필요. `git add -f` 사용.
3. **Rust 툴체인**: 이미 설치됨 (rustc 1.97, cargo, wasm-pack 0.15, wasm32-unknown-unknown 타겟).
4. **WSL 환경**: sudo 필요 시 `SUDO_ASKPASS=/tmp/askpass.sh sudo -A` 사용.
5. **코드 리뷰 스킬**: 반드시 `skill_view(name='requesting-code-review')` 로드 후 실행.
6. **브라우저 테스트**: `browser_navigate` 도구 사용. 테스트 후 서버 종료 필수.
7. **커밋 메시지**: 위 커밋 메시지 그대로 사용. 수정하지 말 것.
8. **Push 후**: GitHub Actions 완료까지 대기 후 Pages 사이트 접속 확인.

## 완료 조건

- [ ] 코드 리뷰 완료 (발견된 문제 수정)
- [ ] `cargo check` 통과
- [ ] `wasm-pack build --target web --release` 통과
- [ ] 브라우저 테스트 통과 (3개 언어 전환 포함)
- [ ] 커밋 완료
- [ ] `git push origin main` 완료
- [ ] GitHub Actions 워크플로 성공
- [ ] GitHub Pages 사이트에서 게임 정상 동작 확인