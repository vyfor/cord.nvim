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
    vim.schedule(function()
        local config = require('cord.plugin.config.util').validate()
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
