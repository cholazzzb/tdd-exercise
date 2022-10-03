function minCost(colors: string, neededTime: number[]): number {
  let res = 0;
  let initColor = colors[0];
  let max = neededTime[0];
  let dif = 0;
  for (let idx = 1; idx < colors.length; idx++) {
    const curColor = colors[idx];
    if (initColor !== curColor) {
      initColor = curColor;
      dif = 0;
      continue;
    }
    if (dif === 0) {
      if (neededTime[idx - 1] < neededTime[idx]) {
        max = neededTime[idx];
        res += neededTime[idx - 1];
      } else {
        max = neededTime[idx - 1];
        res += neededTime[idx];
      }
    } else {
      if (neededTime[idx] > max) {
        res += max;
        max = neededTime[idx];
      } else {
        res += neededTime[idx];
      }
    }
    dif++;
  }
  return res;
}
export { minCost };
