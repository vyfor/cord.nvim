function! CordCompleteList(ArgLead, CmdLine, CmdPos)
    let l:starting_new_arg = a:ArgLead == '' && a:CmdLine[a:CmdPos-1] =~ '\s'
    let l:args = split(a:CmdLine[:a:CmdPos-1], '\s\+')
    
    if len(l:args) <= 1 || (len(l:args) == 2 && a:ArgLead != '')
        let l:commands = luaeval('require("cord.api.command").get_commands()')
        return filter(l:commands, 'stridx(v:val, a:ArgLead) == 0')
    endif
    
    let l:main_cmd = l:args[1]
    
    if l:main_cmd =~ '^enable\|^disable\|^toggle'
        let l:features = luaeval('require("cord.api.command").get_features()')
        return filter(l:features, 'stridx(v:val, a:ArgLead) == 0')
    endif
    
    if len(l:args) == 2 || (len(l:args) == 3 && !l:starting_new_arg)
        let l:subcommands = luaeval('require("cord.api.command").get_subcommands(_A)', l:main_cmd)
        return filter(l:subcommands, 'stridx(v:val, a:ArgLead) == 0')
    endif
    
    return []
endfunction

command! -nargs=+ -complete=customlist,CordCompleteList Cord lua require'cord.api.command'.handle('<q-args>')

lua << EOF
    if vim.g.cord_defer_startup == true then return end

    -- Schedule initialization to next event loop iteration.
    -- This ensures setup() calls have an effect even if called after this file is sourced.
    -- Also allows the plugin to start automatically without requiring setup() call.
    -- This is experimental, do let us know if you're having any issues.
    vim.schedule(function()
        local config = require('cord.api.config').verify()
        if not config then return end

        if config.enabled then
            vim.cmd [[
                augroup Cord
                    autocmd!
                    autocmd VimLeavePre * lua require 'cord.server':cleanup()
                augroup END
            ]]
            
            require('cord.server'):initialize()
        end
    end)
EOF
