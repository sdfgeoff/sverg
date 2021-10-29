PYTHON = python3.9
VENV_START = ${PYTHON} -m venv .env; source .env/bin/activate;


go: build run


setup:
	${VENV_START} \
	pip install -r requirements.txt; \

build:
	${VENV_START} \
	cd painter-core/painter-core; \
	maturin develop
	
run:
	${VENV_START} \
	${PYTHON} painter

fmt:
	cd painter-core; cargo fmt

test:
	cd painter-core; cargo test
