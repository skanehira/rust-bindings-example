build-py:
	cd python && virtualenv venv && source ./venv/bin/activate && cd ../lib && maturin develop
run-py: build-py
	cd python && source ./venv/bin/activate && python main.py
