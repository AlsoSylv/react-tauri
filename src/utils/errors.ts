class InvalidRequestState extends Error {
  invalidFields: { field: string; errorCode: number }[] = [];

  constructor(message: string, invalidFields: { field: string; errorCode: number }[]) {
    super(message);
    this.invalidFields = invalidFields;
  }
}

export default InvalidRequestState;
