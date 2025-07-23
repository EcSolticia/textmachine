function Header (h)
  if h.identifier ~= '' then
    local anchor_link = pandoc.Link(
      "📌",
      '#' .. h.identifier,
      '',
      {class = 'anchor'}
    )
    table.insert(h.content, anchor_link)
    return h
  end
end
