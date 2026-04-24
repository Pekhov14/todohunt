package scanner

import (
	"fmt"
	"io/fs"
	"path/filepath"
	"strings"

	"github.com/go-enry/go-enry/v2"
)

type Scanner struct {
	IgnoreDirs map[string]struct{}
}

func NewScanner() *Scanner {
	// todo: move to config file
	return &Scanner{
		IgnoreDirs: map[string]struct{}{
			".git":         {},
			".idea":        {},
			".vscode":      {},
			"vendor":       {},
			"node_modules": {},
			"dist":         {},
			"build":        {},
			"target":       {},
		},
	}
}

func (s *Scanner) Scan() error {
	fmt.Println("scanning...")
	root := "./"

	err := filepath.WalkDir(root, func(path string, dir fs.DirEntry, err error) error {
		if err != nil {
			return nil // skip problems file
		}

		if dir.IsDir() {
			if strings.HasPrefix(dir.Name(), ".") && dir.Name() != "." && dir.Name() != ".." {
				return filepath.SkipDir
			}

			if _, shouldSkip := s.IgnoreDirs[dir.Name()]; shouldSkip {
				return filepath.SkipDir
			}

			if enry.IsVendor(path) {
				return filepath.SkipDir
			}

			return nil
		}

		fmt.Println("dir:", path, dir.Name())

		return nil
	})

	if err != nil {
		return fmt.Errorf("scan error: %w", err)
	}

	return nil
}
