local Async = require 'cord.core.async'
local Process = require 'cord.core.uv.process'
local logger = require 'cord.api.log'
local utils = require 'cord.core.util'
local uv = vim.uv or vim.loop

local manager
local BASE_URL = 'http://ws.audioscrobbler.com/2.0/?method='
local LASTFM_LOGO =
'https://us1.discourse-cdn.com/flex021/uploads/lastfm/optimized/2X/f/f1f8c34ea6f18aad5b8f89905e38ba7b3424d9b1_2_512x512.png'
local LASTFM_DEFAULT_IMAGE =
'https://lastfm.freetls.fastly.net/i/u/300x300/2a96cbd8b46e442fc41c2b86b821562f.png'

local credentials

local M = {
  config = {
    interval = 10000,
    max_retries = 3,
    override = true,
    fallback_image = LASTFM_LOGO,
  },
  current_track = nil,
}

local request = Async.wrap(function(endpoint, params)
  local url = BASE_URL .. endpoint .. '&format=json' .. credentials
  if params then
    for k, v in pairs(params) do
      url = url .. '&' .. k .. '=' .. v
    end
  end

  local res = Process.spawn({
    cmd = 'curl',
    args = {
      url,
      '--fail',
      '--silent',
      '--show-error',
      '--retry',
      tostring(M.config.max_retries),
    },
  }):unwrap()

  if res.code ~= 0 then
    local err_msg = 'Process exited with code ' .. res.code
    if res.stderr and res.stderr ~= '' then err_msg = err_msg .. '. stderr:\n' .. res.stderr end
    error(err_msg, 0)
  end

  local ok, data = pcall(vim.json.decode, res.stdout)
  if not ok then error('Error while decoding JSON: ' .. data, 0) end

  return data
end)

M.validate = function(config)
  local cfg = vim.tbl_deep_extend('force', M.config, config)
  cfg.interval = math.max(cfg.interval, 500)

  local username = os.getenv 'LASTFM_USERNAME'
  if not username or username == '' then return nil, 'LASTFM_USERNAME is not set' end

  local api_key = os.getenv 'LASTFM_API_KEY'
  if not api_key or api_key == '' then return nil, 'LASTFM_API_KEY is not set' end

  credentials = '&user=' .. utils.url_encode(username) .. '&api_key=' .. utils.url_encode(api_key)

  return cfg
end

M.fetch_track = Async.wrap(function()
  local recent_tracks = request('user.getrecenttracks', {
    limit = 1,
    extended = 1,
  }):unwrap()
  logger.trace(function() return 'LastFM: Received data:\n' .. vim.inspect(recent_tracks) end)

  local track
  local tracks = recent_tracks.recenttracks and recent_tracks.recenttracks.track
  if tracks and #tracks > 0 then
    track = tracks[1]
    local attr = track['@attr']
    if not attr.nowplaying then track = nil end
  end

  if not track then
    logger.debug 'LastFM: Skipping update'
    return
  end

  local title = track.name
  if not title or title == '' then title = 'Unknown' end

  local artist = (track.artist and track.artist.name)
  if not artist or artist == '' then artist = 'Unknown' end

  local album = (track.album and track.album['#text'])
  if not album or album == '' then album = nil end

  local url = (track.url ~= '') and track.url or nil

  local image
  local images = track.image
  if images and #images > 0 then
    image = images[#images]['#text'] ~= '' and images[#images]['#text'] or M.config.fallback_image
  else
    image = M.config.fallback_image
  end

  if image == LASTFM_DEFAULT_IMAGE then image = M.config.fallback_image end

  return {
    title = title,
    artist = artist,
    album = album,
    url = url,
    image = image,
  }
end)

local compare_tracks = function(a, b)
  return a.title == b.title and a.artist == b.artist and a.album == b.album
end

M.run = Async.wrap(function()
  local timer = uv.new_timer()
  timer:start(
    0,
    M.config.interval,
    vim.schedule_wrap(function()
      Async.run(function()
        local track = M.fetch_track():unwrap()
        if not track then
          logger.debug 'LastFM: No track found'
          M.current_track = nil
          if M.config.override then manager:clear_activity() end
          return
        end

        if M.current_track and compare_tracks(M.current_track, track) then
          logger.debug 'LastFM: Track unchanged'
          return
        end
        M.current_track = track

        if M.config.override then
          manager:set_activity({
            type = 'listening',
            status_display_type = 'state',
            details = track.title,
            state = track.artist,
            assets = {
              large_image = track.image,
              large_text = track.album,
              large_url = track.url,
            },
            timestamps = {
              start = os.time(),
            },
          }, true)
        else
          manager:queue_update(true)
        end
      end)
    end)
  )
end)

M.setup = function(config)
  local res, err = M.validate(config)
  if not res then
    error(err, 0)
  else
    M.config = res
  end

  Async.run(function()
    M.run():catch(function(err2) logger.error('LastFM: ' .. err2) end)
  end)

  return {
    name = 'LastFM',
    description = 'Display information about your Last.fm activity',
    variables = {
      track_title = function() return M.current_track and M.current_track.title or nil end,
      track_artist = function() return M.current_track and M.current_track.artist or nil end,
      track_album = function() return M.current_track and M.current_track.album or nil end,
      track_url = function() return M.current_track and M.current_track.url or nil end,
      track_image = function() return M.current_track and M.current_track.image or nil end,
    },
    hooks = {
      ready = {
        function(mngr)
          manager = mngr
          if M.config.override then
            mngr:skip_update()
            mngr:pause()
          end
        end,
        priority = require('cord.internal.hooks').PRIORITY.HIGHEST,
      },
    },
  }
end

return M
