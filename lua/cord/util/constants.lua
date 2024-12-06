local CLIENT_IDS = {
  vim = {
    id = '1219918645770059796',
    icon = 'vim',
  },
  neovim = {
    id = '1219918880005165137',
    icon = 'neovim',
  },
  lunarvim = {
    id = '1220295374087000104',
    icon = 'lunarvim',
  },
  nvchad = {
    id = '1220296082861326378',
    icon = 'nvchad',
  },
  astronvim = {
    id = '1230866983977746532',
    icon = 'astronvim',
  },
}

-- Increment this only when an existing icon is modified
local ASSETS_VERSION = '1'
local ASSETS_URL =
  'https://raw.githubusercontent.com/vyfor/icons/master/icons/flat'

return {
  CLIENT_IDS = CLIENT_IDS,
  ASSETS_URL = ASSETS_URL,
  ASSETS_VERSION = ASSETS_VERSION,
}
