if exists("b:current_syntax")
	finish
end

syn keyword scraptKeyword global vars lists broadcasts costumes sounds
syn keyword scraptMediaTypes SVG PNG WAV MP4
syn keyword scraptConditional if else
syn keyword scraptRepeat repeat
syn keyword scraptBoolean true false
syn match scraptNumber "-\=\<[0-9]*\>"
syn match scraptFloat "-\=\<[0-9]*\.[0-9]*\>"
syn region scraptString start=+"+ end=+"+
syn region scraptComment start="//" end="\n"
syn region scraptMetaComment start="///" end="\n"

hi def link scraptKeyword Keyword
hi def link scraptMediaTypes Type
hi def link scraptRawIdent Identifier
hi def link scraptConditional Conditional
hi def link scraptRepeat Repeat
hi def link scraptBoolean Boolean
hi def link scraptNumber Number
hi def link scraptFloat Float
hi def link scraptString String
hi def link scraptComment Comment
hi def link scraptMetaComment SpecialComment

let b:current_syntax = "scrapt"
