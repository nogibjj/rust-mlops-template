install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	#python -m pytest -vv --cov=mylib testing/

profile-test-code:
	#python -m pytest -vv --durations=1 --durations-min=1.0

parallel-test:
	#python -m pytest -n auto --dist loadgroup -vv --cov=mylib testing/ 

format:
	black $$(git ls-files '*.py')

develop:
	maturin develop

lint:
	pylint --disable=R,C $$(git ls-files '*.py')

all: install lint parallel-test format