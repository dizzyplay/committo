# Committo

커밋 메시지 생성기 

## 빌드

```bash
cargo build --release
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
committo generate     # 실제 API 호출
committo dev          # 드라이런 (API 호출 안함)
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

```bash
# 개발 중 테스트 (현재 폴더)
cargo run -- dev

# 실제 생성 (현재 폴더)  
cargo run -- generate

# 스모크 테스트 (/tmp 격리 환경)
./scripts/smoke_test.sh
```

## 설정 파일

`~/committo.toml`:
```toml
api-key = "your-key-here"
candidate-count = 5
llm-provider = "openai"
llm-model = "gpt-3.5-turbo"
committo-dev = false
```

## 초기 설정

처음 사용시 설정 파일이 없으면 자동으로 대화형 설정 진행:

```bash
$ committo generate
No configuration file found at: /Users/user/committo.toml
Let's set up your configuration interactively!

=== Committo Configuration Setup ===
Enter your OpenAI API key: sk-...
Select LLM provider: [openai]
Select model: [gpt-3.5-turbo, gpt-4]
Number of commit message candidates (5): 
Enable development mode (dry-run by default)? [y/N]: 

✅ Configuration saved to: /Users/user/committo.toml
```

## 예시

```bash
$ git add src/lib.rs
$ committo generate
🔄 Retry (generate new messages)
feat: 환경변수 파싱에 정규표현식 검증 추가
refactor: 설정 파일 로딩 로직 중앙화
Select a commit message: feat: 환경변수 파싱에 정규표현식 검증 추가
```