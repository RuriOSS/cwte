all :
	cargo build --release
	cp target/release/cwte .
fmt :
	clang-format -i --assume-filename=t.c test.hce
	sed -i "s/:</_CE_PAN/g" test.ce
	sed -i "s/::}/_CE_NUS/g" test.ce
	sed -i "s/:D/_CE_LAF/g" test.ce
	clang-format -i --assume-filename=t.c test.ce
	sed -i "s/_CE_PAN/:</g" test.ce
	sed -i "s/_CE_NUS/::}/g" test.ce
	sed -i "s/_CE_LAF/:D/g" test.ce

	sed -i "s/:</_CE_PAN/g" seccomp.ce
	clang-format -i --assume-filename=t.c seccomp.ce
	sed -i "s/_CE_PAN/:</g" seccomp.ce

test:
	cargo build
	cp target/debug/cwte .