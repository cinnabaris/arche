package rss

import (
	"fmt"
	"io"
	"time"

	"github.com/gorilla/feeds"
)

// New new RSS
func New() *RSS {
	return &RSS{
		handlers: make([]Handler, 0),
	}
}

// Handler rss handler
type Handler func(l string) ([]*feeds.Item, error)

// RSS rss helper
type RSS struct {
	handlers []Handler
}

// Register register handler
func (p *RSS) Register(handlers ...Handler) {
	p.handlers = append(p.handlers, handlers...)
}

// Walk walk feed items
func (p *RSS) Walk(l string, f func(...*feeds.Item) error) error {
	for _, hnd := range p.handlers {
		items, err := hnd(l)
		if err != nil {
			return err
		}
		if err = f(items...); err != nil {
			return err
		}
	}
	return nil
}

// ToAtom write to atom
func (p *RSS) ToAtom(host, lang, title, dest string, author *feeds.Author, wrt io.Writer) error {
	now := time.Now()
	feed := &feeds.Feed{
		Title:       title,
		Link:        &feeds.Link{Href: fmt.Sprintf("%s/?locale=%s", host, lang)},
		Description: dest,
		Author:      author,
		Created:     now,
		Items:       make([]*feeds.Item, 0),
	}

	if err := p.Walk(lang, func(items ...*feeds.Item) error {
		feed.Items = append(feed.Items, items...)
		return nil
	}); err != nil {
		return nil
	}

	return feed.WriteAtom(wrt)
}
