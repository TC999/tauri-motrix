const UNITS = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"] as const;

export const parseByteVo = (num?: number | string, suffix = "") => {
  if (typeof num === "string") {
    num = Number(num);
  }
  if (typeof num !== "number" || isNaN(num)) return ["NaN", ""];
  if (num < 1000) return [`${Math.round(num)}`, "B" + suffix];
  const exp = Math.min(Math.floor(Math.log2(num) / 10), UNITS.length - 1);
  const dat = num / Math.pow(1024, exp);
  const ret = dat >= 1000 ? dat.toFixed(0) : dat.toPrecision(3);
  const unit = UNITS[exp] + suffix;

  return [ret, unit];
};

export const toByte = (num: number | string, unit: (typeof UNITS)[number]) => {
  if (typeof num === "string") {
    num = Number(num);
  }
  if (typeof num !== "number" || isNaN(num)) return NaN;
  const exp = UNITS.indexOf(unit);
  if (exp < 0) return NaN;
  return Math.round(num * Math.pow(1024, exp));
};

export const parseByte = (
  num: number | string,
  unit: (typeof UNITS)[number],
) => {
  if (typeof num === "string") {
    num = Number(num);
  }
  if (typeof num !== "number" || isNaN(num)) return NaN;
  const exp = UNITS.indexOf(unit);
  if (exp < 0) return NaN;
  return Math.round(num / Math.pow(1024, exp));
};
