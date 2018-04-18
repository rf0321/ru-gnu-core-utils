class Api::V1::StandardController < ApplicationController
  def index
    time = Bustime.all
    render json: time
  end
  def show
    time = Bustime.where(id:params[:id])
    render json: time
  end
end