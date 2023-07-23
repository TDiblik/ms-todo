// Ensure compile-time correctness when working with dynamic properties
export function nameof<T>(name: keyof T) {
  return name;
}

// Efficient way to count elements, when needed (use inside loops and stuff, where perf matters)
export function count_elements_equal(arr: any[], member: string, desired_value: any) {
  let count = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i][member] == desired_value) {
      count++;
    }
  }
  return count;
}
export function count_elements_not_equal(arr: any[], member: string, desired_value: any) {
  let count = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i][member] != desired_value) {
      count++;
    }
  }
  return count;
}
