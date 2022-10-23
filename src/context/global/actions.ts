/* eslint-disable no-unused-vars */
// eslint-disable-next-line no-shadow
enum Actions {
  UPDATE_ROLE = 'UPDATE_ROLE',
  UPDATE_RANK = 'UPDATE_RANK',
  UPDATE_REGION = 'UPDATE_REGION',
  UPDATE_CHAMPION = 'UPDATE_CHAMPION',
}

type GlobalActions =
  | {
      type: Actions.UPDATE_CHAMPION;
      payload: string;
    }
  | {
      type: Actions.UPDATE_RANK;
      payload: string;
    }
  | {
      type: Actions.UPDATE_REGION;
      payload: string;
    }
  | {
      type: Actions.UPDATE_ROLE;
      payload: string;
    };

export { Actions, GlobalActions };
