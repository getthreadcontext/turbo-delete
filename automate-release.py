import shutil
import os

# Pre-requisites: iscc.exe (Inno Setup), pyinstaller.exe (PyInstaller)

# Compile turbo-delete.exe
os.system('cargo build --profile release-optimized')

# Compile register-context-menu.py
os.system(r'pyinstaller --onefile --distpath temp_dist dist\register-context-menu.py')

# Compile unregister-context-menu.py
os.system(r'pyinstaller --onefile --distpath temp_dist dist\unregister-context-menu.py')

shutil.move(r'temp_dist\register-context-menu.exe', r'bin\register-context-menu.exe')
shutil.move(r'temp_dist\unregister-context-menu.exe', r'bin\unregister-context-menu.exe')

# Copy turbo-delete.exe from the target/release-optimized folder
shutil.copyfile(r'target\release-optimized\turbo-delete.exe', r'bin\td.exe')

# Try to find and run Inno Setup Compiler
import subprocess

def find_iscc():
    """Find the Inno Setup Compiler executable"""
    possible_paths = [
        r'C:\Program Files (x86)\Inno Setup 6\ISCC.exe',
        r'C:\Program Files (x86)\Inno Setup 6\iscc.exe',
        r'C:\Program Files\Inno Setup 6\ISCC.exe',
        r'C:\Program Files\Inno Setup 6\iscc.exe',        r'C:\Program Files (x86)\Inno Setup 5\iscc.exe',
        r'C:\Program Files\Inno Setup 5\iscc.exe',
        'iscc.exe'  # Try if it's in PATH
    ]
    
    for path in possible_paths:
        try:
            result = subprocess.run([path, '/?'], capture_output=True)
            if result.returncode in [0, 1]:  # Both 0 and 1 are valid for help commands
                return path
        except (subprocess.CalledProcessError, FileNotFoundError):
            continue
    return None

iscc_path = find_iscc()
if iscc_path:
    os.system(f'"{iscc_path}" dist\\turbo-delete.iss')
    print("Setup installer created successfully!")
else:
    print("WARNING: Inno Setup Compiler (iscc.exe) not found!")
    print("Please install Inno Setup from https://jrsoftware.org/isinfo.php")
    print("Or manually run: iscc.exe dist\\turbo-delete.iss")

# Cleanup
if os.path.exists('register-context-menu.spec'):
    os.remove('register-context-menu.spec')
if os.path.exists('unregister-context-menu.spec'):
    os.remove('unregister-context-menu.spec')
if os.path.exists('build'):
    shutil.rmtree(r'build')
if os.path.exists('temp_dist'):
    shutil.rmtree(r'temp_dist')
if os.path.exists(r'dist\__pycache__'):
    shutil.rmtree(r'dist\__pycache__')
