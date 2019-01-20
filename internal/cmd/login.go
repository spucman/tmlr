package cmd

import (
	"github.com/spf13/cobra"
	"github.com/spucman/tmlr/internal/http-api"
)

func createLoginCommand() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "login",
		Short: "Logs in and prints the JWT token",
		Long:  `Logs in and prints or stores the JWT token`,
	}

	var apiKey, apiSecret string
	cmd.Flags().StringVarP(&apiKey, "apiKey", "", "", "API Key (required)")
	cmd.MarkFlagRequired("apiKey")

	cmd.Flags().StringVarP(&apiSecret, "apiSecret", "", "", "API Secret (required)")
	cmd.MarkFlagRequired("apiSecret")

	cmd.Run = func(cmd *cobra.Command, args []string) {
		httpapi.Login(apiKey, apiSecret)
	}

	return cmd
}
