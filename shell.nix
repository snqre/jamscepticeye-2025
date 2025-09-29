{ pkgs ? import <nixpkgs> {} }: pkgs.mkShell {
	buildInputs = [
		pkgs.rustc
		pkgs.cargo
		pkgs.pkg-config
		pkgs.alsa-lib
		pkgs.libxkbcommon
		pkgs.vulkan-loader
		pkgs.vulkan-headers
		pkgs.udev
		pkgs.xorg.libX11
		pkgs.xorg.libXcursor
		pkgs.xorg.libXrandr
		pkgs.xorg.libXi
		pkgs.xorg.libXrender
		pkgs.xorg.libxcb
		pkgs.libGL
		pkgs.dbus
		pkgs.mesa
	];

	shellHook = ''
		export WINIT_UNIX_BACKEND=wayland
		export LD_LIBRARY_PATH="${pkgs.alsa-lib.out}/lib:${pkgs.libGL.out}/lib:${pkgs.libxkbcommon.out}/lib:$LD_LIBRARY_PATH"
		echo "✅ WINIT_UNIX_BACKEND set to wayland"
	'';
}