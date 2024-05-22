local function file_exists(filename)
  local stat = vim.loop.fs_stat(filename)
  return stat and stat.type == 'file'
end

local function move_file(src, dest)
  local result, err = os.rename(src, dest)
  if not result then
    vim.api.nvim_err_writeln('[cord.nvim] Error moving file: ' .. err)
  end
end

local function init_discord(ffi)
  local cord_file
  local os_name = vim.loop.os_uname().sysname
  if os_name:find('Windows', 1, true) == 1 then -- starts with 'Windows'
    cord_file = '/cord.dll'
  elseif os_name == 'Linux' then
    cord_file = '/cord.so'
  elseif os_name == 'Darwin' then
    cord_file = '/cord.dylib'
  else
    vim.api.nvim_err_writeln '[cord.nvim] Unable to identify OS type'
  end

  local path = debug.getinfo(2, 'S').source:sub(2, -14)
  local old_path = path .. '/cord'
  local new_path = path .. cord_file
  if file_exists(old_path) then
    os.remove(new_path)
    move_file(old_path, new_path) -- move file as to avoid file access errors when updating
  end

  ffi.cdef [[
    typedef struct {
      const char* client;
      const char* image;
      const char* editor_tooltip;
      const char* idle_text;
      const char* idle_tooltip;
      const char* viewing_text;
      const char* editing_text;
      const char* file_browser_text;
      const char* plugin_manager_text;
      const char* lsp_manager_text;
      const char* workspace_text;
      const char* initial_path;
      const bool swap;
    } InitArgs;
    typedef struct {
      const char* filename;
      const char* filetype;
      const char* cursor_position;
      int problem_count;
      bool is_read_only;
    } PresenceArgs;
    typedef struct {
      const char* first_label;
      const char* first_url;
      const char* second_label;
      const char* second_url;
    } Buttons;
    void init(
      const InitArgs* args,
      const Buttons* buttons
    );
    const bool update_presence(
      const PresenceArgs* args
    );
    const bool update_presence_with_assets(
      const char* name,
      const char* icon,
      const char* tooltip,
      int asset_type,
      const PresenceArgs* args
    );
    void clear_presence();
    void disconnect();
    void set_workspace(const char* workspace);
    const char* update_workspace(const char* workspace);
    void update_time();
    const char* get_workspace();
  ]]

  return ffi.load(new_path)
end

local function validate_severity(config)
  config.lsp.severity = tonumber(config.lsp.severity)
  if
    config.lsp.severity == nil
    or config.lsp.severity < 1
    or config.lsp.severity > 4
  then
    vim.api.nvim_err_writeln '[cord.nvim] config.lsp.severity value must be a number between 1 and 4'
    return false
  end
  return true
end

local function get_problem_count(config)
  if config.lsp.show_problem_count then
    local bufnr = config.lsp.scope == 'buffer'
        and vim.api.nvim_get_current_buf()
      or nil
    if bufnr == nil and config.lsp.scope ~= 'workspace' then
      vim.api.nvim_err_writeln '[cord.nvim] config.lsp.scope value must be either workspace or buffer'
    end
    return #vim.diagnostic.get(
      bufnr,
      { severity = { min = config.lsp.severity } }
    )
  end
end

local function array_contains(arr, val)
  if arr == nil or val == nil then return false end

  for _, value in ipairs(arr) do
    if value == val then return true end
  end

  return false
end

local function get_file_extension(filename)
  for i = #filename, 1, -1 do
    if filename:sub(i, i) == '.' then return filename:sub(i) end
  end

  return filename
end

local function get_icon(config, filename, filetype)
  if not config.assets then return end

  local icon = config.assets[filetype]
  if icon then return icon, filetype end

  icon = config.assets[filename]
  if icon then return icon, filename end

  local extension = get_file_extension(filename)
  icon = config.assets[extension]
  if icon then return icon, extension end
end

return {
  init_discord = init_discord,
  validate_severity = validate_severity,
  get_problem_count = get_problem_count,
  array_contains = array_contains,
  get_icon = get_icon,
}
