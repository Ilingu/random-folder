package main

import (
	"context"
	"crypto/rand"
	"errors"
	"math/big"
	"os"
	"path"
	"path/filepath"

	"github.com/skratchdot/open-golang/open"
	"github.com/wailsapp/wails/v2/pkg/runtime"
)

// App struct
type App struct {
	ctx        context.Context
	folder     string
	subdirs    []string
	lastWinner string
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

// Init Session by asking where is the folder to pick from
func (a *App) SetFolder() string {
	dirPath, err := runtime.OpenDirectoryDialog(a.ctx, runtime.OpenDialogOptions{Title: "Choose a directory containing sub-dirs", CanCreateDirectories: true})
	if err != nil {
		return Response[string]{Succeed: false, Reason: "couldn't open dialog"}.marshal()
	}
	if dirPath == "" || !filepath.IsAbs(dirPath) {
		return Response[string]{Succeed: false, Reason: "invalid path"}.marshal()
	}

	a.folder = dirPath
	return Response[string]{Succeed: true, Data: dirPath}.marshal()
}

func (a *App) GetRandomSubdirPath() string {
	if a.folder == "" {
		return Response[string]{Succeed: false, Reason: "no root folder"}.marshal()
	}

	if len(a.subdirs) == 0 {
		err := a.LoadsSubdirs()
		if err != nil {
			return Response[string]{Succeed: false, Reason: "no subdirectories found"}.marshal()
		}
	}

	res, err := rand.Int(rand.Reader, big.NewInt(int64(len(a.subdirs))))
	if err != nil {
		return Response[string]{Succeed: false, Reason: "couldn't generate random value"}.marshal()
	}

	idx := res.Uint64()
	randSubname := a.subdirs[idx]
	a.subdirs = append(a.subdirs[:idx], a.subdirs[idx+1:]...)

	randPath := path.Join(a.folder, randSubname)
	var firstImage string

	entries, err := os.ReadDir(randPath)
	if err == nil {
		for _, e := range entries {
			if !e.IsDir() && isImage(e.Name()) {
				firstImage = e.Name()
				break
			}
		}
	}

	a.lastWinner = randPath
	return Response[[]string]{Succeed: true, Data: []string{randPath, firstImage}}.marshal()
}

func (a *App) LoadsSubdirs() error {
	if a.folder == "" {
		return errors.New("no root folder")
	}

	entries, err := os.ReadDir(a.folder)
	if err != nil {
		return err
	}

	if len(entries) == 0 {
		return errors.New("no sub folders")
	}

	allSubdirs := []string{}
	for _, e := range entries {
		if e.IsDir() {
			allSubdirs = append(allSubdirs, e.Name())
		}
	}

	a.subdirs = allSubdirs
	return nil
}

func (a *App) OpenWinner() string {
	if a.lastWinner == "" || !filepath.IsAbs(a.lastWinner) {
		return Response[string]{Succeed: false, Reason: "no winner"}.marshal()
	}

	err := open.Run(a.lastWinner)
	if err != nil {
		return Response[string]{Succeed: false, Reason: "error when opening the path"}.marshal()
	}
	return Response[string]{Succeed: true}.marshal()

}

func (a *App) ResetFolder() {
	a.folder = ""
	a.subdirs = []string{}
	a.lastWinner = ""
}
