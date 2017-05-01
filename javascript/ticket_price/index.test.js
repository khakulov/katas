const calculate = require('./index');

test('returns 0 if no days given', () => {
  expect(calculate([])).toBe(0);
});

test('returns 2 for one day', () => {
  expect(calculate([1])).toBe(2);
});

test('returns 6 for three days', () => {
  expect(calculate([1,2,3])).toBe(6);
});

test('returns 7 for 4 days in one week', () => {
  expect(calculate([1,2,3,4])).toBe(7);
});

test('returns 7 for days in one week', () => {
  expect(calculate([1,2,3,4])).toBe(7);
  expect(calculate([1,2,3,4,5])).toBe(7);
  expect(calculate([1,2,3,4,5,6])).toBe(7);
  expect(calculate([1,2,3,4,5,6,7])).toBe(7);
  expect(calculate([2,3,4,5,6,7])).toBe(7);
  expect(calculate([2,3,4,6,7])).toBe(7);
  expect(calculate([1,3,5,7])).toBe(7);
});

test('returns 9 for days in one week + 1 day', () => {
  expect(calculate([1,2,3,4,8])).toBe(9);
  expect(calculate([1,2,3,4,5,8])).toBe(9);
  expect(calculate([1,2,3,4,5,6,8])).toBe(9);
  expect(calculate([1,2,3,4,5,6,7,8])).toBe(9);
  expect(calculate([1,2,3,4,5,6,8])).toBe(9);
  expect(calculate([1,2,3,4,6,8])).toBe(9);
  expect(calculate([1,2,3,5,8])).toBe(9);
});

test('returns 7 for days in one week + x day', () => {
  expect(calculate([1,2,3,4,8,9])).toBe(11);
  expect(calculate([1,2,3,4,8,9,10])).toBe(13);
  expect(calculate([1,2,3,4,8,9,10,11])).toBe(14);
});


test('returns 18 for 2 weeks and 2 days', () => {
  expect(calculate([1,5,6,7,8,9,10,11,12,16,17,18,19,20])).toBe(18);
});


test('returns 25 for one month', () => {
  expect(calculate([1,3,5,7,9,11,13,15,17,19,21,23,25,27,29])).toBe(25);
});