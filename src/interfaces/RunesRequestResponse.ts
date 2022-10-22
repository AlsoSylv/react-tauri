type RunesRequestResponse =
  | {
      runes: string[][];
      completedSuccessfully: true;
    }
  | {
      message: string;
      completedSuccessfully: false;
    };

export default RunesRequestResponse;
