package cmd

import (
	"github.com/spf13/cobra"
)

//CreateRootCommand creates the cobra root command for this application
func CreateRootCommand() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "tmlr",
		Short: "tmlr is a client for Timeular",
		Long: `tmlr is a client to use the OpenAPI of timeular to use the most 
					  common funtions.`,
		Run: func(cmd *cobra.Command, args []string) {
			cmd.HelpFunc()
		},
	}

	registerSubCommands(cmd)

	return cmd
}

func registerSubCommands(cmd *cobra.Command) {
	cmd.AddCommand(createVersionCommand(), createLoginCommand())
}
