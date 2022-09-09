# 本地启动服务
local:
	@cargo run --bin launcher --features local

run: local

test:
	@cargo test
