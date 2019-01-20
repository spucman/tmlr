package main

import (
	"fmt"
	"os"

	"github.com/mitchellh/go-homedir"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"github.com/spucman/tmlr/internal/cmd"
)

func createInitializeFunc(cfgFile string) func() {
	return func() {
		readCfg := false

		if cfgFile != "" {
			viper.SetConfigFile(cfgFile)
			readCfg = true
		} else {
			home, err := homedir.Dir()
			if err != nil {
				return
			}

			if _, err := os.Stat(fmt.Sprintf("%s/.tmlr", home)); err == nil {
				viper.AddConfigPath(home)
				viper.SetConfigName(".tmlr")
				readCfg = true
			}
		}

		if readCfg {
			if err := viper.ReadInConfig(); err != nil {
				fmt.Println("Can't read config:", err)
				os.Exit(1)
			}
		}
	}
}

func main() {
	var cfgFile string

	rootCmd := cmd.CreateRootCommand()
	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.tmlr.yaml)")
	viper.BindPFlag("config", rootCmd.PersistentFlags().Lookup("config"))

	rootCmd.PersistentFlags().StringVar(&cfgFile, "token", "", "personal access token for the timeular app")
	viper.BindPFlag("token", rootCmd.PersistentFlags().Lookup("token"))

	cobra.OnInitialize(createInitializeFunc(cfgFile))

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
