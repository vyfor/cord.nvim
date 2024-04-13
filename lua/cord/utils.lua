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
    vim.api.nvim_err_writeln('[cord.nvim] Unable to identify OS type')
  end

  local path = debug.getinfo(2, 'S').source:sub(2, -14)
  local old_path = path .. '/cord'
  local new_path = path .. cord_file
  if file_exists(old_path) then
    os.remove(new_path)
    move_file(old_path, new_path) -- move file as to avoid file access errors when updating
  end

  ffi.cdef[[
    void init(
      const char* client,
      const char* image,
      const char* editorTooltip,
      const char* idleText,
      const char* idleTooltip,
      const char* viewingText,
      const char* editingText,
      const char* fileBrowserText,
      const char* pluginManagerText,
      const char* workspaceText
    );
    bool update_presence(
      const char* filename,
      const char* filetype,
      bool isReadOnly,
      const char* cursorPosition,
      int problemCount
    );
    void clear_presence();
    void disconnect();
    void set_cwd(const char* directory);
    void set_buttons(
      const char* first_label,
      const char* first_url,
      const char* second_label,
      const char* second_url
    );
    void update_time();
  ]]

  return ffi.load(new_path)
end

local function fetch_repository()
  local handle = io.popen('git config --get remote.origin.url')
  if not handle then
    vim.notify('[cord.nvim] Could not fetch Git repository URL', vim.log.levels.WARN)
    return
  end
  local git_url = handle:read('*a')
  handle:close()

  return git_url:match('^%s*(.-)%s*$')
end

local function find_workspace()
  local curr_dir = vim.fn.expand('%:p:h')
  local vcs_markers = {'.git', '.svn', '.hg'}

  while curr_dir ~= '' do
    for _, dir in ipairs(vcs_markers) do
      if vim.fn.isdirectory(curr_dir .. '/' .. dir) == 1 then
        return vim.fn.fnamemodify(curr_dir, ':t')
      end
    end

    curr_dir = vim.fn.fnamemodify(curr_dir, ':h')
    if curr_dir == vim.fn.fnamemodify(curr_dir, ':h') then break end -- reached root
  end

  return vim.fn.fnamemodify(vim.fn.getcwd(), ':t') -- fallback to cwd
end

local function validate_severity(config)
  config.lsp.severity = tonumber(config.lsp.severity)
  if config.lsp.severity == nil or config.lsp.severity < 1 or config.lsp.severity > 4 then
    vim.api.nvim_err_writeln('[cord.nvim] config.lsp.severity value must be a number between 1 and 4')
    return false
  end
  return true
end

local function validate_buttons(config)
  if config.display.show_repository then
    local buttons = {}
    local repo
    for i, button in ipairs(config.buttons) do
      if i > 2 then
        vim.notify('[cord.nvim] Detected more than two buttons in the config. Only the first two will be displayed', vim.log.levels.WARN)
        return buttons
      end
      if button.url == 'git' then
        if not repo then
          repo = fetch_repository()
        end
        if repo and repo ~= '' then
          table.insert(buttons, { label = button.label, url = repo })
        end
      else
        table.insert(buttons, button)
      end
    end
    return buttons
  end
end

local function update_cwd(config, discord)
  discord.set_cwd(find_workspace())

  local buttons = validate_buttons(config)
  if not buttons then return end

  if #buttons == 1 then
    discord.set_buttons(buttons[1].label, buttons[1].url, nil, nil)
  elseif #buttons >= 2 then
    discord.set_buttons(buttons[1].label, buttons[1].url, buttons[2].label, buttons[2].url)
  end
end

local function get_problem_count(config)
  if config.lsp.show_problem_count then
    local bufnr = config.lsp.scope == 'buffer' and vim.api.nvim_get_current_buf() or nil
    if bufnr == nil and config.lsp.scope ~= 'workspace' then
      vim.api.nvim_err_writeln('[cord.nvim] config.lsp.scope value must be either workspace or buffer')
    end
    return #vim.diagnostic.get(bufnr, { severity = { min = config.lsp.severity } })
  end
end

return {
  init_discord = init_discord,
  fetch_repository = fetch_repository,
  find_workspace = find_workspace,
  validate_severity = validate_severity,
  update_cwd = update_cwd,
  get_problem_count = get_problem_count
}
