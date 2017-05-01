const calculate = require('./index');

test('returns 0 if no days given', () => {
  expect(calculate([])).toBe(0);
});