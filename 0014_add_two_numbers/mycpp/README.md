# C++ Notes

I am also using [Conan](https://docs.conan.io/2/tutorial/creating_packages/create_your_first_package.html).
This package was made with the following command:

```bash
conan new cmake_lib -d name=twosum -d version=1.0
```

Conan was not available for some reason.
Did the following:

```bash
pip show conan # Showed it was installed
sudo ln -s ~/.local/bin/conan /usr/bin/conan
```

That set it up so the command line could use it again.

You can build the library with:

```bash
conan build ./conanfile.py
```

That will put stuff in the build directory.
We also want to test.

You can run:

```bash
conan create .
```

This creates the build directory in test_package.
There's also:

```bash
conan test test_package addtwo/1.0
```

They both will execute tests.
