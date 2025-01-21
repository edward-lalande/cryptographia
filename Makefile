##
## EPITECH PROJECT, 2024
## my_pgp
## File description:
## Makefile
##

NAME = my_pgp

BIN_PATH = target/release/$(NAME)

all: $(NAME)

$(NAME):
	cargo build --offline --release && cp $(BIN_PATH) $(NAME)

clean:
	cargo clean
	rm -rf cargo-test*

fclean: clean
	rm -f $(NAME)

tests_run:
	cargo +stable install cargo-llvm-cov --locked
	cargo llvm-cov --no-report

coverage:
	cargo llvm-cov

re: fclean all

.PHONY: all re clean fclean tests_run coverage
