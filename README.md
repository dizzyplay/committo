# Committo

커밋 메시지 생성기 

## install(Build)

```bash
build
cargo build --release
```

## install(Homebrew)
```bash
brew tap dizzyplay/committo
brew install committo

brew update
brew upgrade committo
```
## 사용법

### 설정
```bash
# 설정 값 지정
committo set api-key 'your-key-here'
committo set candidate-count 5
committo set llm-model gpt-4

# 설정 확인
committo show
```

### 커밋 메시지 생성
```bash
git add .
committo or committo generate   # 실제 API 호출
committo generate --dry-run          # 드라이런 (프롬프트 확인용. API 호출 안함)
```

## 컨벤션 파일

`.committoconvention` 파일로 계층적 커밋 규칙 정의:

```bash
# 홈 디렉토리 - 개인 취향
echo "간결하고 명확한 한글 커밋 메시지 선호" > ~/.committoconvention

# 프로젝트 루트 - 프로젝트 전체 규칙  
echo "conventional commits 형식: feat/fix/docs/refactor" > /project/.committoconvention

# 모노레포 패키지 - 세부 컨벤션
echo "frontend: UI 컴포넌트 변경시 component: 접두사 사용" > /project/frontend/.committoconvention
```

**프롬프트 결합 순서:** 부모 → 자식 디렉토리 순으로 합쳐져 더 구체적이고 맥락에 맞는 커밋 메시지 생성

## 개발

# 스모크 테스트 (/tmp 격리 환경)
```
./scripts/smoke_test.sh
```

## 초기 설정

처음 사용시 설정 파일이 없으면 자동으로 대화형 설정 진행:

```bash
$ committo
No configuration file found at: /Users/user/committo.toml
...
```

## 예시

```bash
$ git add src/lib.rs
$ committo
🔄 Retry (generate new messages)
feat: 환경변수 파싱에 정규표현식 검증 추가
refactor: 설정 파일 로딩 로직 중앙화
Select a commit message: feat: 환경변수 파싱에 정규표현식 검증 추가
```
