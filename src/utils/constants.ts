const DEFAULT_CACHE_DURATION = 1000 * 60 * 60 * 24; // 1 DAY

const errors: Record<number, { errorCode: number; error: string; message: string }> = {
  101: {
    errorCode: 101,
    error: 'Data missing',
    message: 'Data for combo does not exist',
  },
  102: {
    errorCode: 102,
    error: 'Networking Error',
    message: 'Cannot connect to required server',
  },
  103: {
    errorCode: 103,
    error: 'Champ Missing',
    message: 'Champion does not exist according to Riot',
  },
  104: {
    errorCode: 104,
    error: 'Data Dragon Missing',
    message: 'Data Dragon is missing or not updated',
  },
  105: {
    errorCode: 105,
    error: 'CDragon Missing',
    message: 'All asset servers in use are down*',
  },
  106: {
    errorCode: 106,
    error: 'Role is Invalid',
    message: 'The role value entered has no data, or is invalid',
  },
  201: {
    errorCode: 201,
    error: 'Overview Missing',
    message: 'UGG Overview JSON is missing',
  },
  202: {
    errorCode: 202,
    error: 'Overview Connect Error',
    message: 'UGG Overview URL changed',
  },
  203: {
    errorCode: 203,
    error: 'Ranking Missing',
    message: 'UGG Ranking JSON is missing',
  },
  204: {
    errorCode: 204,
    error: 'Ranking Connect Error',
    message: 'UGG Ranking URL changed',
  },
  205: {
    errorCode: 205,
    error: 'Rate Error',
    message: 'The Corresponding rate has no data',
  },
  206: {
    errorCode: 206,
    error: 'Matches error',
    message: 'There are no matches for the selected combo',
  },
  207: {
    errorCode: 207,
    error: 'No Ability Order',
    message: 'Overview does not contain abilities for combo',
  },
  208: {
    errorCode: 208,
    error: 'Role Missing',
    message: 'Role JSON is missing',
  },
  209: {
    errorCode: 209,
    error: 'Role Connect Error',
    message: 'Role URL has changed',
  },
  401: {
    errorCode: 401,
    error: 'LCU connect error',
    message: 'No client was found to connect to',
  },
  402: {
    errorCode: 402,
    error: 'Cannot delete pages',
    message: 'No pages were able to be deleted',
  },
  403: {
    errorCode: 403,
    error: 'Cannot create pages',
    message: 'Was not able to create page',
  },
  404: {
    errorCode: 404,
    error: 'Cannot get page',
    message: 'Cannot get current page',
  },
  405: {
    errorCode: 405,
    error: 'Pushed successfully',
    message: 'Everything worked properly',
  },
};

export { DEFAULT_CACHE_DURATION, errors };
