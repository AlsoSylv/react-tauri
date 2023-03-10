interface CachedData<T> {
  validUntil: number;
  data: T;
}

export default CachedData;
