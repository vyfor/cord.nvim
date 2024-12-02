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

local ASSETS_URL =
  'https://raw.githubusercontent.com/vyfor/cord.nvim/master/assets'
local ASSETS_VERSION = '16'

return {
  CLIENT_IDS = CLIENT_IDS,
  ASSETS_URL = ASSETS_URL,
  ASSETS_VERSION = ASSETS_VERSION,
}
