package sitemap

import (
	"compress/gzip"
	"io"

	"github.com/ikeikeikeike/go-sitemap-generator/stm"
)

// Handler sitemap handler
type Handler func() ([]stm.URL, error)

// New new sitemap
func New() *Sitemap {
	return &Sitemap{
		handlers: make([]Handler, 0),
	}
}

// Sitemap sitemap helper
type Sitemap struct {
	handlers []Handler
}

// Register register handler
func (p *Sitemap) Register(handlers ...Handler) {
	p.handlers = append(p.handlers, handlers...)
}

// ToXMLGz write xml.gz
func (p *Sitemap) ToXMLGz(h string, w io.Writer) error {
	sm := stm.NewSitemap()
	sm.Create()
	sm.SetDefaultHost(h)
	for _, hnd := range p.handlers {
		items, err := hnd()
		if err != nil {
			return err
		}
		for _, it := range items {
			sm.Add(it)
		}
	}
	buf := sm.XMLContent()

	wrt := gzip.NewWriter(w)
	defer wrt.Close()
	wrt.Write(buf)
	return nil
}
