from PyInstaller.building.build_main import \
    Analysis, \
    EXE, \
    PYZ
import glob


block_cipher = None


a = Analysis(
    ["isopy_bin/main.py"],
    pathex=[],
    binaries=[],
    datas=[],
    hiddenimports=[],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False)


def get_data_files(pattern):
    data_files = []
    for f in glob.glob(pattern):
        data_files.append((f, f, "DATA"))

    return data_files


a.datas += get_data_files("sha256sums/*.sha256sums")

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name="isopy",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None)
