import subprocess

def test_run_ex1():
    result = subprocess.run(["python", "./examples/ex1.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex2():
    result = subprocess.run(["python", "./examples/ex2.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex3():
    result = subprocess.run(["python", "./examples/ex3.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex4():
    result = subprocess.run(["python", "./examples/ex4.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex5():
    result = subprocess.run(["python", "./examples/ex5.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex6():
    result = subprocess.run(["python", "./examples/ex6.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex7():
    result = subprocess.run(["python", "./examples/ex7.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex8():
    result = subprocess.run(["python", "./examples/ex8.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex9():
    result = subprocess.run(["python", "./examples/ex9.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex10():
    result = subprocess.run(["python", "./examples/ex10.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0

def test_run_ex11():
    result = subprocess.run(["python", "./examples/ex11.py"], capture_output=True, text=True)
    print("Standard Output:", result.stdout)
    print("Standard Error:", result.stderr)
    assert result.returncode == 0


