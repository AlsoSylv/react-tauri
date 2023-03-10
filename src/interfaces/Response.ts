type Response<T> =
  | {
      Ok: T;
    }
  | {
      Err: number;
    };

export default Response;
