set dotenv-load

_default:
	just --list

# Fetches the input and creates the project files for a given `DAY`
add DAY: (_fetch DAY)
	./add_day.sh {{DAY}}

# Runs a given `DAY`
run DAY:
	cargo run -r --bin day`printf "%02d" {{DAY}}`

_fetch DAY:
	curl 'https://adventofcode.com/2022/day/{{DAY}}/input' \
		-H "cookie: session=$SESSION_TOKEN" -o "inputs/day`printf "%02d" {{DAY}}`.txt"

# Tests a given `DAY`
test DAY:
	cargo test day`printf "%02d" {{DAY}}`

report:
	open ./target/criterion/report/index.html
