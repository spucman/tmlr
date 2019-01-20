package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

func createVersionCommand() *cobra.Command {
	return &cobra.Command{
		Use:   "version",
		Short: "Print the version number of tmlr",
		Long:  `Print the version of tmlr`,
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("tmlr v0.1")
		},
	}
}
