# Example

## summer.cpp

```cpp
#include <iostream>
double summer(double arr[], int size) {
    double sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}
```

## summerMain.cpp

```cpp
#include <iostream>
// include the google test dependencies
#include <gtest/gtest.h>
// declare the function(s) that you are testing
double summer(double[], int);
// our first unit test
TEST(IntegerInputsSuite, simpleSum) {
    // first, set up any inputs to your
    const int SIZE = 3;
    double arr[SIZE] = {1, 2, 3};
    // then, make an assertion to test
    EXPECT_EQ(summer(arr, SIZE), 6) << "The sum is not correct";
}
TEST(IntegerInputsSuite, oneElement) {
    const int SIZE = 1;
    double arr[SIZE] = {33};
    EXPECT_EQ(summer(arr, SIZE), 33) << "The sum is not correct for array of size 1";
}
TEST(DoubleInputsSuite, simpleSum) {
    const int SIZE = 3;
    double arr[SIZE] = {1.1, 1.1, 1};
    EXPECT_EQ(summer(arr, SIZE), 3.2) << "The sum is not correct using double inputs";
}
int main(int argc, char **argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
```

## To Build Test

```bash
g++ -std=c++0x summer.cpp summerMain.cpp -lgtest -lgtest_main -pthread -o sumProgram
```

Note that in the future, you’ll want to replace `summer.cpp` and `summerMain.cpp` with whatever files you are compiling. In addition, replace `sumProgram` with whatever you want the compiled program to be called.

## Run The Test

```bash
./sumProgram
```
