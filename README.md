# Committo

커밋 메시지 생성기 

## 빌드

```bash
cargo build --release
```

## 사용법

### 설정
```bash
# API 키 설정
committo env set OPENAI_API='your-key'

# 설정 확인
committo env show
```

### 커밋 메시지 생성
```bash
git add .
committo generate     # 실제 API 호출
committo dev          # 드라이런 (API 호출 안함)
```

## 컨벤션 파일

`.comittoconvention` 파일로 계층적 커밋 규칙 정의:

```bash
# 홈 디렉토리 - 개인 취향
echo "간결하고 명확한 한글 커밋 메시지 선호" > ~/.comittoconvention

# 프로젝트 루트 - 프로젝트 전체 규칙  
echo "conventional commits 형식: feat/fix/docs/refactor" > /project/.comittoconvention

# 모노레포 패키지 - 세부 컨벤션
echo "frontend: UI 컴포넌트 변경시 component: 접두사 사용" > /project/frontend/.comittoconvention
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

`~/.comittoorc`:
```bash
export OPENAI_API="your-key-here"
```

## 예시

```bash
$ git add src/lib.rs
$ committo generate
feat: 환경변수 파싱에 정규표현식 검증 추가
```