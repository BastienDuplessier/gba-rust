default: out xargo-build objcopy gba-fix

clean:
	rm -rf target
	rm -rf out
	rm Cargo.lock

out:
	mkdir -p out

xargo-build:
	xargo build --target=gba --release

objcopy:
	arm-none-eabi-objcopy -O binary target/gba/release/first out/game.gba

gba-fix:
	gbafix out/game.gba
