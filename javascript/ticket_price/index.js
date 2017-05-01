function calculate(A) {
  if (!A.length) {
    return 0;
  }

  const [currentDay, ...R] = A;
  const price1 = 2 + calculate(R);
  const price2 = 7 + calculate(R.filter(d => d > currentDay + 6));
  const minPrice = price1 < price2 ? price1 : price2;
  return minPrice > 25 ? 25 : minPrice;
}
module.exports = calculate;